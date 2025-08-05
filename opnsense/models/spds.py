from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Spds(BaseModel):
    class Meta:
        name = "SPDs"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
