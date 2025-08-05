from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class MmonitUrl(BaseModel):
    class Meta:
        name = "mmonitUrl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
