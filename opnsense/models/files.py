from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Files(BaseModel):
    class Meta:
        name = "files"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
