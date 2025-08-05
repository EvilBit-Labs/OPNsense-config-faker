from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.disablepreempt import Disablepreempt
from opnsense.models.disconnectppps import Disconnectppps
from opnsense.models.password import Password
from opnsense.models.pfsyncinterface import Pfsyncinterface
from opnsense.models.pfsyncpeerip import Pfsyncpeerip
from opnsense.models.pfsyncversion import Pfsyncversion
from opnsense.models.synchronizetoip import Synchronizetoip
from opnsense.models.syncitems import Syncitems
from opnsense.models.username import Username

__NAMESPACE__ = "https://opnsense.org/config"


class Hasync(BaseModel):
    class Meta:
        name = "hasync"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    disablepreempt: Disablepreempt = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    disconnectppps: Disconnectppps = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pfsyncinterface: Pfsyncinterface = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pfsyncpeerip: Pfsyncpeerip = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pfsyncversion: Pfsyncversion = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    synchronizetoip: Synchronizetoip = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    username: Username = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    password: Password = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    syncitems: Syncitems = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
