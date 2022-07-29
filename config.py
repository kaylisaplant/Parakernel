# -----------------------------------------------------------------------------
from os.path import join, abspath, realpath, dirname
PARAVIEW_INSTALL_DIR =        "/shft/app/paraview/build"
PARAVIEW_PYTHON_DIR =         join(PARAVIEW_INSTALL_DIR, "lib/python3.9/site-packages")
PARAVIEW_CLIENT_EXECUTABLE =  join(PARAVIEW_INSTALL_DIR, "bin/paraview")
PARAVIEW_SERVER_EXECUTABLE =  join(PARAVIEW_INSTALL_DIR, "bin/pvserver")
PARAVIEW_QT_PLUGIN_NAME =     "PVQtJupyterPlugin"
PARAVIEW_QT_PLUGIN_PATH =     join(dirname(realpath(__file__)), PARAVIEW_QT_PLUGIN_NAME)

# -----------------------------------------------------------------------------
IPARAVIEW_PRELUDE = """\
# -----------------------------------------------------------------------------
# Import needed package
import sys
sys.path.append('%s')
from paraview.simple import *
from paraview import collaboration
import asyncio
from types import SimpleNamespace

# -----------------------------------------------------------------------------
# Create the variable we'll need and store them in a specific place in the globals
__pv = SimpleNamespace()

# -----------------------------------------------------------------------------
# Connect to server
__pv.connection = Connect('{}', {}, timeout=0)

# -----------------------------------------------------------------------------
# Setup collaboration manager
__pv.collaboration_manager = servermanager.ActiveConnection.Session.GetCollaborationManager()
__pv.collaboration_manager.PromoteToMaster(__pv.collaboration_manager.GetUserId())
__pv.collaboration_manager.SetUserLabel("IParaView Kernel")
__pv.collaboration_manager.UpdateUserInformations()

# -----------------------------------------------------------------------------
# Callback on events to sync up on each chat message for the trace
__pv.trace = ""
def _on_collaboration_message(obj, *args, **kwargs):
  global __pv
  __pv.trace = __pv.collaboration_manager.GetLastReceivedMessage()

__pv.collaboration_manager.AddObserver(servermanager.vtkSMCollaborationManager.LastMessageUpdated, _on_collaboration_message)

# -----------------------------------------------------------------------------
# Create default layout
__pv.default_view = GetActiveViewOrCreate('RenderView')
__pv.layout = GetLayout()
__pv.layout.SetSize(800, 600)

# -----------------------------------------------------------------------------
# Periodically fetch server events
async def _periodic_callback(interval, function):
  while True:
    await asyncio.gather(asyncio.sleep(interval), function())

async def _collaboration_server_callback():
  collaboration.processServerEvents()

# this line cannot be executed in the prelude because there is no running event loop yet.
# _server_loop = asyncio.create_task(_periodic_callback(2, _collaboration_server_callback))

""" % PARAVIEW_PYTHON_DIR

# -----------------------------------------------------------------------------
IPARAVIEW_HELP = """\
# IParaView Kernel

Custom IPython kernel for an easy interface with ParaView.

## Magic commands:

  - `%help` : get this help
  - `%paraview` : open a Qt client connecting to the current kernel server. Return the client ID.
  - `%sync <int>` : synchronize the Notebook camera with the given client ID.
If no id is given then all Qt clients will be synchronized with the jupyter client.
  - Every others magic command available from your usual ipython interpreter.

## Accessible variables:

If you want to have full access to the underlying objects, see namespace `__pv`.
Here's some usefull attributes of the namespace :

  - `connection` : holds the connection information of the kernel to the ParaView server
  - `default_view` : a default render view created for ease-of-use sake.
  - `layout` : a default view layout created for ease-of-use sake.
  - `trace` : the last recorded trace from a qt client.

**WARNING**: do not touch these variables unless you know what you're doing

## Using the generated Python trace from the GUI

By default the GUI cannot be used, as only one client (either the python client or the GUi one) can control the server.
To do some action in the GUI, click the red circle button (record button) in the toolbar.
Once you are done, click the record button (now green) again.
A python editor will pop up in the Qt client, showing you the trace of what you've done.
The same trace is accessible directly from the variable `__pv.trace` in your jupyter environment.
It is possible to use this variable in some interesting way through the builtin magic commands.
Here's 2 possible usage :

 - `%pycat __pv.trace` : display the trace through a syntax-highlighted pager.
 - `%load __pv.trace` : load the trace into the current cell
"""
