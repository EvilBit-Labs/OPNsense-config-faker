from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Msgcachesize(BaseModel):
    class Meta:
        name = "msgcachesize"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
