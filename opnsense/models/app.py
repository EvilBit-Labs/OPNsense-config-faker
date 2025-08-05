from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

__NAMESPACE__ = "https://opnsense.org/config"


class App(BaseModel):
    class Meta:
        name = "app"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    value: int = field(
        metadata={
            "required": True,
        }
    )
