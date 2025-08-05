from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Prv(BaseModel):
    class Meta:
        name = "prv"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: bytes = field(
        metadata={
            "required": True,
            "format": "base64",
        }
    )
