from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Prefetchkey(BaseModel):
    class Meta:
        name = "prefetchkey"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
