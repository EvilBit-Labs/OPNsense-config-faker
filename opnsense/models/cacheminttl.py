from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Cacheminttl(BaseModel):
    class Meta:
        name = "cacheminttl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
