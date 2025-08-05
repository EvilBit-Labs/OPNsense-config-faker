from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Dhcrelay(BaseModel):
    class Meta:
        name = "DHCRelay"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
