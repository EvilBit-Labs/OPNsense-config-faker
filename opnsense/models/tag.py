from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Tag(BaseModel):
    class Meta:
        name = "tag"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
