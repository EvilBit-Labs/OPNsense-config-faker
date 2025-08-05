from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Cachemaxnegativettl(BaseModel):
    class Meta:
        name = "cachemaxnegativettl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
