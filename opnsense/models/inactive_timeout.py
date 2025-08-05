from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class InactiveTimeout(BaseModel):
    class Meta:
        name = "inactiveTimeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: int = field(
        metadata={
            "required": True,
        }
    )
