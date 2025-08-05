from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Poolopts(BaseModel):
    class Meta:
        name = "poolopts"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
