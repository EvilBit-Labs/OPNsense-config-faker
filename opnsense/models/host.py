from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Host(BaseModel):
    class Meta:
        name = "host"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
