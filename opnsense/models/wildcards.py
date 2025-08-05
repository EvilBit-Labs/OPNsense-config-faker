from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Wildcards(BaseModel):
    class Meta:
        name = "wildcards"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
