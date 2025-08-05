from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Events(BaseModel):
    class Meta:
        name = "events"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
