from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Rrsetcachesize(BaseModel):
    class Meta:
        name = "rrsetcachesize"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
