from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Keepalive(BaseModel):
    class Meta:
        name = "keepalive"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
