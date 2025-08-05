from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class PooloptsSourcehashkey(BaseModel):
    class Meta:
        name = "poolopts_sourcehashkey"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
