from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.servers import Servers

__NAMESPACE__ = "https://opnsense.org/config"


class OpenVpnexport(BaseModel):
    class Meta:
        name = "OpenVPNExport"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    servers: Servers = field(
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
