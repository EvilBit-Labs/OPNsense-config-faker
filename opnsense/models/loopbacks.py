from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Loopbacks(BaseModel):
    class Meta:
        name = "loopbacks"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
