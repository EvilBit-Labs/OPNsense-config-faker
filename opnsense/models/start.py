from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Start(BaseModel):
    class Meta:
        name = "start"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
