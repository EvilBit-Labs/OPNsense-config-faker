from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Groups(BaseModel):
    class Meta:
        name = "groups"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
