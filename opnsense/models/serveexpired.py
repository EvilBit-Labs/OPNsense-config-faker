from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serveexpired(BaseModel):
    class Meta:
        name = "serveexpired"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
