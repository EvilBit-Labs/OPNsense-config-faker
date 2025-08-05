from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Networks(BaseModel):
    class Meta:
        name = "networks"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
