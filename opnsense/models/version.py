from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class Version(BaseModel):
    class Meta:
        name = "version"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: str = field(
        default="",
        metadata={
            "required": True,
        },
    )
