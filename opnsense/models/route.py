from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Route(BaseModel):
    class Meta:
        name = "route"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
