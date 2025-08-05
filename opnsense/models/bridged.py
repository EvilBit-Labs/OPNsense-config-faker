from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Bridged(BaseModel):
    class Meta:
        name = "bridged"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
