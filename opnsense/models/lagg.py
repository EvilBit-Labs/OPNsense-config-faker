from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Lagg(BaseModel):
    class Meta:
        name = "lagg"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
