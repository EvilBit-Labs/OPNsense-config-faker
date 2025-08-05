from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Destinations(BaseModel):
    class Meta:
        name = "destinations"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
