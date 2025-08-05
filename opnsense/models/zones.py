from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Zones(BaseModel):
    class Meta:
        name = "zones"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
