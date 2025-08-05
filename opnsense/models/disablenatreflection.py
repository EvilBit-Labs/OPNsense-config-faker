from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Disablenatreflection(BaseModel):
    class Meta:
        name = "disablenatreflection"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: str = field(
        default="",
        metadata={
            "required": True,
        },
    )
