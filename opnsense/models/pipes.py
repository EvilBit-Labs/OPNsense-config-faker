from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Pipes(BaseModel):
    class Meta:
        name = "pipes"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
