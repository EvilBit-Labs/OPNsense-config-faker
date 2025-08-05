from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serveexpiredreplyttl(BaseModel):
    class Meta:
        name = "serveexpiredreplyttl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
