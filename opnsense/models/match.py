from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Match(BaseModel):
    class Meta:
        name = "match"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
