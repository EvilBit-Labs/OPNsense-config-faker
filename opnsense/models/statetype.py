from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Statetype(BaseModel):
    class Meta:
        name = "statetype"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: str = field(
        default="",
        metadata={
            "required": True,
        },
    )
