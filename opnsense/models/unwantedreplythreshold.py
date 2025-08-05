from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Unwantedreplythreshold(BaseModel):
    class Meta:
        name = "unwantedreplythreshold"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
