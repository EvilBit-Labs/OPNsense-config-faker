from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Onetoone(BaseModel):
    class Meta:
        name = "onetoone"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
