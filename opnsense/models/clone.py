from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Clone(BaseModel):
    class Meta:
        name = "clone"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
