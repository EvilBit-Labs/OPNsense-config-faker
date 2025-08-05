from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Pidfile(BaseModel):
    class Meta:
        name = "pidfile"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
