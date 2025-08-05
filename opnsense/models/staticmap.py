from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.dnsserver import Dnsserver
from opnsense.models.hostname import Hostname
from opnsense.models.ipaddr import Ipaddr
from opnsense.models.mac import Mac
from opnsense.models.ntpserver import Ntpserver
from opnsense.models.winsserver import Winsserver

__NAMESPACE__ = "https://opnsense.org/config"


class Staticmap(BaseModel):
    class Meta:
        name = "staticmap"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    mac: Mac = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ipaddr: Ipaddr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    hostname: Hostname = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    winsserver: Winsserver = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dnsserver: Dnsserver = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    ntpserver: Ntpserver = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
