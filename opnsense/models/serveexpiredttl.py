from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serveexpiredttl(BaseModel):
    class Meta:
        name = "serveexpiredttl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
