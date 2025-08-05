from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Targets(BaseModel):
    class Meta:
        name = "targets"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
