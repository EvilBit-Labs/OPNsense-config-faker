from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Templates(BaseModel):
    class Meta:
        name = "templates"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
