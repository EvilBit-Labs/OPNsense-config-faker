from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Vtis(BaseModel):
    class Meta:
        name = "VTIs"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
