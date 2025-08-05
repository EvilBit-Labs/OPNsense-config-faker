from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.gateway_item import GatewayItem

__NAMESPACE__ = "https://opnsense.org/config"


class Gateways2(BaseModel):
    class Meta:
        name = "gateways"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    gateway_item: GatewayItem = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
