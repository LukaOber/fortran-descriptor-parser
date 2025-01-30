{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  pre-commit.hooks = {
    nixfmt-rfc-style.enable = true;
    nixfmt-rfc-style.package = pkgs.nixfmt-rfc-style;
    taplo.enable = true;
    yamllint.enable = true;
    rustfmt = {
      enable = true;
      always_run = true;
    };
    clippy = {
      enable = true;
      always_run = true;
      settings.denyWarnings = true;
      settings.allFeatures = true;
    };
  };
}
