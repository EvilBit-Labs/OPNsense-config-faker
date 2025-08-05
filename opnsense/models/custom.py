from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Custom(BaseModel):
    class Meta:
        name = "custom"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
