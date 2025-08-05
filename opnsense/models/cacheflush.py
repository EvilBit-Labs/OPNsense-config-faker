from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Cacheflush(BaseModel):
    class Meta:
        name = "cacheflush"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
