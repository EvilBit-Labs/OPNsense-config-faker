from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Remotes(BaseModel):
    class Meta:
        name = "remotes"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
