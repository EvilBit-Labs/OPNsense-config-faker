from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.enabled import Enabled
from opnsense.models.max_unacked_clients import MaxUnackedClients
from opnsense.models.this_server_name import ThisServerName

__NAMESPACE__ = "https://opnsense.org/config"


class Ha(BaseModel):
    class Meta:
        name = "ha"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enabled: Enabled = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    this_server_name: ThisServerName = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    max_unacked_clients: MaxUnackedClients = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
