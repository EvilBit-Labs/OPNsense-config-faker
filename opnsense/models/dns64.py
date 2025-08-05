from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Dns64(BaseModel):
    class Meta:
        name = "dns64"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
