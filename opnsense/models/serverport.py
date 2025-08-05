from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Serverport(BaseModel):
    class Meta:
        name = "serverport"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
