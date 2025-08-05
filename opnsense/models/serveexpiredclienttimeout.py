from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serveexpiredclienttimeout(BaseModel):
    class Meta:
        name = "serveexpiredclienttimeout"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
