import atexit
import os
import random
import subprocess
from typing import Tuple
from typing import Dict
from ipykernel.kernelapp import IPKernelApp
from ipykernel.ipkernel import IPythonKernel
from IPython import display

# Setup global constant strings for paraview install directories,
# IPARAVIEW_PRELUDE and IPARAVIEW_HELP
from config import *

# -----------------------------------------------------------------------------
global_launched_process = []

def clear_processes() -> None:
  """
  Clear all processes registered in 'global_launched_process' global array
  """

  for process in global_launched_process:
    if process.poll() is None:
      process.kill()
  global_launched_process.clear()

atexit.register(clear_processes)

# -----------------------------------------------------------------------------
class IParaViewKernel(IPythonKernel):
  implementation = 'IParaView'
  implementation_version = '1.0'
  language = 'pvpython'
  language_info = {'name': 'python',
                  'mimetype': 'text/x-python',
                  'file_extension': '.py'}
  banner = 'ParaView Jupyter - interactive python interface for paraview'

  def do_shutdown(self, *args, **kwargs) -> Dict[str,str]:
    """
    Override of super class 'do_shutdown' for clearing all processes at shutdown.
    """

    clear_processes()
    return super().do_shutdown(*args, **kwargs)

# -----------------------------------------------------------------------------
def launch_paraview_server(logger) -> Tuple[subprocess.Popen, str, int]:
  """
  launch_paraview_server(logger) -> (server process, host, port)

  Launch the ParaView server. Read the output of the process and blocks until
  the server is not ready to accept connections.
  If the environment contains the variable `IPARAVIEW_PORT_RANGE` then it will
  be used to determine the range of available port for the server.
  Default range is [11000, 50000].
  """

  host = "localhost"
  portRange = (11000, 50000)
  if "IPARAVIEW_PORT_RANGE" in os.environ:
    try:
      portRange = tuple(map(int, os.getenv("IPARAVIEW_PORT_RANGE").split(",")))
      assert len(portRange) == 2
    except:
      logger.warning("IPARAVIEW_PORT_RANGE syntax is wrong, ignoring")

  port = random.randint(*portRange)
  command = [PARAVIEW_SERVER_EXECUTABLE, "--multi-clients", "--server-port", str(port)]
  process = subprocess.Popen(command, stdout=subprocess.PIPE, universal_newlines=True)
  global_launched_process.append(process)

  if process is not None:
    for line in process.stdout:
      if line.startswith('Accepting connection'):
        break

  return process, host, port

# -----------------------------------------------------------------------------
class ParaviewClientLauncher:
  """
  %paraview -> int

  Open a Qt client of ParaView connecting to the server launched by the current jupyter kernel.
  Return the client id of the created instance. This id can then be used to synchronize the
  camera of the jupyter kernel with a specific client (see magic %sync).

  On first call this will also initialize the server loop on jupyter side so the
  python interpreter is able to react to events sended by Qt clients
  """

  host = "localhost"
  port = 11111
  app = None
  _counter = 1

  def __call__(self, _line) -> int:
    command = [PARAVIEW_CLIENT_EXECUTABLE,
      "--server-url"         , "cs://{}:{}".format(self.host, self.port),
      "--plugin-search-paths", PARAVIEW_QT_PLUGIN_PATH,
      "--plugins"            , PARAVIEW_QT_PLUGIN_NAME]
    process = subprocess.Popen(command)
    global_launched_process.append(process)

    if (ParaviewClientLauncher._counter == 1 and self.app):
      # This is needed here because it cannot be executed in the prelude. Reason is as long
      # as the kernel is not running (ie app.run() has not been called), there is no event loop
      res = self.app.shell.run_cell("__pv.server_loop = asyncio.create_task(_periodic_callback(2, _collaboration_server_callback))")
      res.raise_error()

    # XXX here we assume the client id we always be incrementing by step of one.
    # Possible improvement : create a client plugin that outputs in stdout the
    # actual client id, read it from there (like in launch_paraview_server) and return it
    ParaviewClientLauncher._counter = ParaviewClientLauncher._counter + 1
    return ParaviewClientLauncher._counter

# -----------------------------------------------------------------------------
def help(line) -> display.Markdown:
  """
  %help -> display.Markdown

  Print help about the IParaView kernel.
  """

  return display.Markdown(IPARAVIEW_HELP)

# -----------------------------------------------------------------------------
def main():
  ## Initialisation
  app = IPKernelApp.instance(kernel_class=IParaViewKernel)
  app.name = "IParaView"
  app.initialize()

  # Launch the server and connect to it
  process, host, port = launch_paraview_server(app.log)
  if (process is None or process.poll() is not None):
    app.log.error("Could not launch pvserver")
    return

  # Run prelude to setup the default environment. Make sure to make the kernel fail if
  # the prelude couldn't be executed
  app.shell.run_cell(IPARAVIEW_PRELUDE.format(host, port)).raise_error()

  # Register magic for opening Qt client
  launcher = ParaviewClientLauncher()
  launcher.host = host
  launcher.port = port
  launcher.app = app
  app.shell.register_magic_function(launcher, 'line', 'paraview')

  # Register magic for getting some help
  app.shell.register_magic_function(help, 'line')

  # Register magic for synchronizing jupyter client with a specific one
  def sync(line) -> None:
    """
    %sync(clientId: int) -> None

    Synchronize the kernel camera with the given Qt client ID.
    """

    clientId = line if line.isdigit() else "__pv.collaboration_manager.GetUserId()"
    res = app.shell.run_cell(
      "__pv.collaboration_manager.UpdateUserInformations()\n"
      "__pv.collaboration_manager.FollowUser({})\n"
      "__pv.collaboration_manager.UpdateUserInformations()".format(clientId))
    res.raise_error()

  app.shell.register_magic_function(sync, 'line')

  ## Main Loop
  app.start()

# -----------------------------------------------------------------------------
if __name__ == '__main__':
  main()
